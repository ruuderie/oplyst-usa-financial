import type { Meta, StoryObj } from '@storybook/vue3';

import MenubarRadioItem from '../components/ui/menubar/MenubarRadioItem.vue';

const meta = {
  title: 'MenubarRadioItem',
  component: MenubarRadioItem,
  tags: ['autodocs'],
  args: {}, // default value
} satisfies Meta<typeof MenubarRadioItem>;

export default meta;
type Story = StoryObj<typeof MenubarRadioItem>;

export const Primary: Story = {
    args: {
        class: 'page-container__navbar navbar container is-fluid h-28',
    }
};