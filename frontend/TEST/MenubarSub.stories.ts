import type { Meta, StoryObj } from '@storybook/vue3';

import MenubarSub from '../components/ui/menubar/MenubarSub.vue';

const meta = {
  title: 'MenubarSub',
  component: MenubarSub,
  tags: ['autodocs'],
  args: {}, // default value
} satisfies Meta<typeof MenubarSub>;

export default meta;
type Story = StoryObj<typeof MenubarSub>;

export const Primary: Story = {
    args: {
        class: 'page-container__navbar navbar container is-fluid h-28',
    }
};