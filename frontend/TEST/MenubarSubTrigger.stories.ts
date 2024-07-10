import type { Meta, StoryObj } from '@storybook/vue3';

import MenubarSubTrigger from '../components/ui/menubar/MenubarSubTrigger.vue';

const meta = {
  title: 'MenubarSubTrigger',
  component: MenubarSubTrigger,
  tags: ['autodocs'],
  args: {}, // default value
} satisfies Meta<typeof MenubarSubTrigger>;

export default meta;
type Story = StoryObj<typeof MenubarSubTrigger>;

export const Primary: Story = {
    args: {
        class: 'page-container__navbar navbar container is-fluid h-28',
    }
};